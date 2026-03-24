package grpcapi

import (
	"bufio"
	"encoding/json"
	"fmt"
	"io"
	"os"
	"time"
)

// feedFileEvent mirrors the structure of feed.FeedEvent for JSON parsing.
// We define it here to avoid coupling to unexported feed internals.
type feedFileEvent struct {
	Timestamp string                 `json:"ts"`
	Source    string                 `json:"source"`
	Type      string                 `json:"type"`
	Actor     string                 `json:"actor"`
	Summary   string                 `json:"summary"`
	Payload   map[string]interface{} `json:"payload,omitempty"`
	Count     int                    `json:"count,omitempty"`
}

// readFeedJSONL reads the most recent events from a .feed.jsonl file.
// It reads from the tail of the file to efficiently get recent events.
func readFeedJSONL(path string, limit int) ([]*EventInfo, error) {
	f, err := os.Open(path)
	if err != nil {
		if os.IsNotExist(err) {
			return nil, nil
		}
		return nil, fmt.Errorf("opening feed file: %w", err)
	}
	defer f.Close()

	info, err := f.Stat()
	if err != nil {
		return nil, fmt.Errorf("stat feed file: %w", err)
	}
	if info.Size() == 0 {
		return nil, nil
	}

	// Read at most 1MB from the end of the file.
	const tailSize int64 = 1 << 20
	seekTo := info.Size() - tailSize
	if seekTo < 0 {
		seekTo = 0
	}
	if _, err := f.Seek(seekTo, io.SeekStart); err != nil {
		return nil, fmt.Errorf("seeking feed file: %w", err)
	}

	scanner := bufio.NewScanner(f)
	if seekTo > 0 {
		scanner.Scan() // skip potential partial first line at cut point
	}

	var all []*EventInfo
	for scanner.Scan() {
		var evt feedFileEvent
		if err := json.Unmarshal(scanner.Bytes(), &evt); err != nil {
			continue
		}

		ts, _ := time.Parse(time.RFC3339, evt.Timestamp)

		// Convert payload map[string]interface{} to map[string]string.
		payload := make(map[string]string)
		for k, v := range evt.Payload {
			payload[k] = fmt.Sprintf("%v", v)
		}

		all = append(all, &EventInfo{
			EventType: evt.Type,
			Source:    evt.Source,
			AgentID:  evt.Actor,
			Summary:  evt.Summary,
			Payload:  payload,
			CreatedAt: ts,
		})
	}

	if limit <= 0 {
		limit = 50
	}
	if len(all) > limit {
		all = all[len(all)-limit:]
	}

	return all, nil
}
