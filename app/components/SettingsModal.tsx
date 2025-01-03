import { useState } from 'react';

export default function SettingsModal() {
  const [fontSize, setFontSize] = useState(16);

  return (
    <div>
      <h2>Configurações de Leitura</h2>
      <label>
        Tamanho da Fonte
        <input
          type="range"
          min="12"
          max="24"
          value={fontSize}
          onChange={(e) => setFontSize(Number(e.target.value))}
        />
      </label>
    </div>
  );
}
