import { OnboardingForm } from "../components/OnBoardingForm";

export default function OnboardingPage() {
    return (
        <main className="relative min-h-screen">
            <div className="grid grid-cols-1 lg:grid-cols-2 min-h-screen">
                <div className="m-24 z-10">
                    <header>
                        <h1 className="font-audiowide text-6xl">StudyStudio</h1>
                    </header>
                    <section className="mt-48">
                        <h2 className="text-3xl font-semibold">
                            Create your own.<br />
                            Study Space
                        </h2>
                        <section className="mt-6">
                            <p>
                                Welcome to StudyStudio! We're excited to have you join our community.<br />
                                Before you get started, we need to know a little bit about you.
                            </p>
                        </section>
                    </section>
                </div>

                <section className="flex items-center justify-center p-8 z-10">
                    <OnboardingForm />
                </section>
            </div>

            <img
                src="https://i.imgur.com/BL7rtkh.png"
                alt="Background decoration"
                className="absolute bottom-6 left-24 h-[40rem] z-0"
            />
        </main>
    );
}
