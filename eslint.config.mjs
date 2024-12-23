import { dirname } from "path";
import { fileURLToPath } from "url";
import { FlatCompat } from "@eslint/eslintrc";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const compat = new FlatCompat({
  baseDirectory: __dirname,
});

const eslintConfig = [
  ...compat.extends("next/core-web-vitals", "next/typescript"),
  {
    rules: {
      // Desativa a regra de variáveis não utilizadas
      "@typescript-eslint/no-unused-vars": "off",
      // Desativa a regra de entidades não escapadas
      "react/no-unescaped-entities": "off",
      // Desativa a recomendação de uso de <img> em vez de <Image />
      "@next/next/no-img-element": "off",
    },
  },
];

export default eslintConfig;
