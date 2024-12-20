import { Audiowide, Montserrat } from 'next/font/google';

const audiowide = Audiowide({
    weight: '400',
    subsets: ['latin'],
    display: 'swap',
});

const montserrat = Montserrat({
    weight: ['400', '500', '600', '700'],
    subsets: ['latin'],
    display: 'swap',
});

export default function OnboardingLayout({
    children,
}: {
    children: React.ReactNode;
}) {
    return (
        <div className={`${audiowide.className} ${montserrat.className}`}>
            {children}
        </div>
    );
}