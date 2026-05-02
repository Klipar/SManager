function EmptyState() {
  return (
    <section
      aria-label="Agent details empty state"
      className="flex min-h-[calc(100vh-16rem)] items-center justify-center px-8 pb-8 pt-40 text-center md:pt-56"
    >
      <div>
        <h2 className="text-5xl font-semibold tracking-tight text-white/92">Select Agent</h2>
        <p className="mt-4 text-base text-white/50">
          Choose an agent from the sidebar to continue.
        </p>
      </div>
    </section>
  )
}

export { EmptyState }
