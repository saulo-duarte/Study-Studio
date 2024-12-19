import { ProfileForm } from "../components/LoginForm";

export default function LoginPage() {
  return (
    <div 
      className="border-2 px-12 pb-12 pt-4 m-4 w-96"
      style={{ borderRadius: "1.5rem" }}
      >
      <ProfileForm />
    </div>
  );
}