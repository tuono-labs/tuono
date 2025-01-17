export const blockingAsync = (callback: () => Promise<void>): void => {
  void (async (): Promise<void> => {
    await callback()
  })()
}
