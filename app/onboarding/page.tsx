import { OnboardingForm } from "../components/OnBoardingForm";

export default function OnboardingPage() {
    return (
        <main>
            <header>
                <h1 className="font-audiowide">StudyStudio</h1>
            </header>
            <section>
                <h2>
                    Create your own.<br />
                    Study Space
                </h2>
            </section>
            <section>
                <p>
                    Welcome to StudyStudio! We're excited to have you join our community.<br />
                    Before you get started, we need to know a little bit about you.
                </p>
            </section>
            <section>
                <OnboardingForm />
            </section>
        </main>
    );
}
