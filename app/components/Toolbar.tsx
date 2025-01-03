export default function Toolbar({
    currentPage,
    totalPages,
    onNavigate,
  }: {
    currentPage: number;
    totalPages: number;
    onNavigate: (direction: 'next' | 'prev') => void;
  }) {
    return (
      <div>
        <button disabled={currentPage === 1} onClick={() => onNavigate('prev')}>
          Anterior
        </button>
        <span>{currentPage} / {totalPages}</span>
        <button disabled={currentPage === totalPages} onClick={() => onNavigate('next')}>
          Próxima
        </button>
      </div>
    );
  }
  