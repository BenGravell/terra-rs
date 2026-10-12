export default function initializer() {
  return {
    onStart: () => {},
    onProgress: ({ current, total }) => {
      document.getElementById('loading-detail').textContent = total && current < total
        ? `Loading Terra… ${Math.round(current / total * 100)}%`
        : 'Preparing the map and country rankings…';
    },
    onSuccess: () => {},
    onFailure: error => window.terraFailed(error),
  };
}
