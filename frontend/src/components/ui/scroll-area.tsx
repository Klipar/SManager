import * as React from "react"

import { cn } from "@/lib/utils"

function ScrollArea({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="scroll-area"
      className={cn("overflow-auto overscroll-contain", className)}
      {...props}
    />
  )
}

function ScrollAreaViewport({
  className,
  ...props
}: React.ComponentProps<"div">) {
  return <div className={cn("h-full w-full", className)} {...props} />
}

export { ScrollArea, ScrollAreaViewport }