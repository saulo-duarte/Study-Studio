import { useState, useEffect } from 'react';

// Hook personalizado para verificar se a tela é um dispositivo móvel
export function useIsMobile() {
  // Estado que armazena se a tela é pequena
  const [isMobile, setIsMobile] = useState<boolean>(false);

  // Função para verificar o tamanho da tela
  const checkIsMobile = () => {
    // Considera que a largura máxima da tela para um dispositivo móvel é 768px
    setIsMobile(window.innerWidth <= 768);
  };

  // Hook de efeito para escutar mudanças no tamanho da janela
  useEffect(() => {
    // Verifica o tamanho da tela ao montar o componente
    checkIsMobile();

    // Adiciona event listener para verificar mudanças no tamanho da tela
    window.addEventListener('resize', checkIsMobile);

    // Remove o event listener quando o componente for desmontado
    return () => {
      window.removeEventListener('resize', checkIsMobile);
    };
  }, []); // Este efeito é executado apenas uma vez, quando o componente é montado

  // Retorna o estado que indica se a tela é móvel ou não
  return isMobile;
}
