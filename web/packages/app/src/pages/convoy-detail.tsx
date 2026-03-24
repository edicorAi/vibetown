import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@vibetown/ui/components/card"
import { Badge } from "@vibetown/ui/components/badge"
import { Separator } from "@vibetown/ui/components/separator"
import { useConvoy } from "@vibetown/web-core/hooks/use-orchestration"
import { Button } from "@vibetown/ui/components/button"
import { ArrowLeft } from "lucide-react"

function convoyStatusVariant(
  status: string
): "default" | "secondary" | "outline" | "destructive" {
  switch (status) {
    case "active":
      return "default"
    case "completed":
      return "secondary"
    case "failed":
      return "destructive"
    case "pending":
      return "outline"
    default:
      return "secondary"
  }
}

interface ConvoyDetailPageProps {
  convoyId: string
  onBack: () => void
}

export function ConvoyDetailPage({ convoyId, onBack }: ConvoyDetailPageProps) {
  const convoyQuery = useConvoy(convoyId)
  const convoy = convoyQuery.data

  return (
    <div className="mx-auto max-w-4xl space-y-4">
      <div className="flex items-center gap-3">
        <Button variant="ghost" size="icon-sm" onClick={onBack}>
          <ArrowLeft className="size-4" />
        </Button>
        <h1 className="font-heading text-lg font-semibold">Convoy Detail</h1>
      </div>

      {convoyQuery.isLoading ? (
        <p className="text-sm text-muted-foreground">Loading convoy...</p>
      ) : convoyQuery.isError ? (
        <p className="text-sm text-destructive">Failed to load convoy</p>
      ) : convoy ? (
        <Card>
          <CardHeader>
            <div className="flex items-center gap-3">
              <CardTitle>{convoy.name}</CardTitle>
              <Badge variant={convoyStatusVariant(convoy.status)}>
                {convoy.status}
              </Badge>
            </div>
            <CardDescription>ID: {convoy.id}</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div>
              <h3 className="mb-1.5 text-sm font-medium text-muted-foreground">
                Formula
              </h3>
              <pre className="overflow-x-auto rounded-lg bg-muted p-3 text-xs leading-relaxed">
                {convoy.formula || "(empty)"}
              </pre>
            </div>
            <Separator />
            <dl className="grid grid-cols-[auto_1fr] gap-x-4 gap-y-1 text-sm">
              <dt className="text-muted-foreground">Status</dt>
              <dd className="font-medium">{convoy.status}</dd>
            </dl>
          </CardContent>
        </Card>
      ) : null}
    </div>
  )
}
