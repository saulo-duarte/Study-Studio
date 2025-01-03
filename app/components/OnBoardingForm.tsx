'use client'

import { useState } from "react"
import { useRouter } from "next/navigation"
import { zodResolver } from "@hookform/resolvers/zod"
import { useForm } from "react-hook-form"
import * as z from "zod"
import { invoke } from '@tauri-apps/api/core';

import {
    Form,
    FormControl,    
    FormDescription,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from "./ui/form";
import { Input } from "./ui/input";
import { Button } from "./ui/button";
import { 
    Card, 
    CardHeader, 
    CardTitle, 
    CardDescription,
    CardContent, 
} from "./ui/card"

import { MultiSelect } from "./MultiSelect"

import { useToast } from "@/app/hooks/use-toast"
import { FaReact, FaMobileAlt, FaServer, FaTools } from "react-icons/fa";
import { 
    BarChart, 
    Database, 
    Loader2, 
    Calendar, 
    Sun, 
    Moon 
} from "lucide-react";

const interestingFields = [
    { id: "frontend", name: "Frontend", icon: FaReact },
    { id: "backend", name: "Backend", icon: FaServer },
    { id: "mobile", name: "Mobile", icon: FaMobileAlt },
    { id: "data-science", name: "Data Science", icon: BarChart },
    { id: "data-engineering", name: "Data Engineering", icon: Database },
    { id: "devops", name: "DevOps", icon: FaTools },
];

const availableDays = [
    { id: "Monday", name: "Monday", icon: Calendar },
    { id: "Tuesday", name: "Tuesday", icon: Calendar },
    { id: "Wednesday", name: "Wednesday", icon: Calendar },
    { id: "Thursday", name: "Thursday", icon: Calendar },
    { id: "Friday", name: "Friday", icon: Calendar },
    { id: "Saturday", name: "Saturday", icon: Sun },
    { id: "Sunday", name: "Sunday", icon: Moon },
];

const formSchema = z.object({
    name: z.string()
        .min(2, "Name must be at least 2 characters")
        .max(30, "Name must be at most 30 characters")
        .nonempty("Name is required"),
    email: z.string()
        .email("Invalid email address")
        .nonempty("Email is required"),
    interestingFields: z.array(z.string())
        .nonempty("Please select at least one field"),
    availableDays: z.array(z.string())
        .nonempty("Please select at least one day"),
});

type FormValues = z.infer<typeof formSchema>;

export function OnboardingForm() {
    const router = useRouter();
    const { toast } = useToast();
    const [isLoading, setIsLoading] = useState(false);

    const form = useForm<FormValues>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            name: "",
            email: "",
            interestingFields: [],
            availableDays: [],
        }
    });

    async function onSubmit(data: FormValues) {
        try {
            setIsLoading(true);
    
            if (!data.name?.trim()) {
                throw new Error("Name is required");
            }
    
            await invoke("create_user_command", {
                name: data.name.trim(),
                email: data.email?.trim() || null,
                status: "active",
                availableDays: data.availableDays,
                interests: data.interestingFields,
              });
    
            toast({
                title: "Success!",
                description: "Your account has been created successfully",
                duration: 5000,
            });
            router.push("/");
    
        } catch (error) {
            const backendMessage = error instanceof Error 
                ? error.message 
                : "An error occurred while creating your account. Please try again.";
    
            console.error("Error creating user:", error);
    
            toast({
                variant: "destructive",
                title: "Error",
                description: backendMessage,
                duration: 7000,
            });
        } finally {
            setIsLoading(false);
        }
    }
    

    return (
        <Card className="w-full max-w-md mx-auto">
            <CardHeader>
                <CardTitle>Welcome to Study Studio!</CardTitle>
                <CardDescription>Tell a little bit about you</CardDescription>
            </CardHeader>
            <CardContent>
                <Form {...form}>
                    <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">
                        <FormField
                            control={form.control}
                            name="name"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>Name</FormLabel>
                                    <FormControl>
                                        <Input
                                            placeholder="Your name"
                                            {...field}
                                            disabled={isLoading}
                                        />
                                    </FormControl>
                                    <FormDescription>How should we call you?</FormDescription>
                                    <FormMessage />
                                </FormItem>
                            )}
                        />
                        <FormField
                            control={form.control}
                            name="email"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>Email</FormLabel>
                                    <FormControl>
                                        <Input
                                            placeholder="youremail@email.com"
                                            {...field}
                                            disabled={isLoading}
                                        />
                                    </FormControl>
                                    <FormDescription>We'll never share your email with anyone else.</FormDescription>
                                    <FormMessage />
                                </FormItem>
                            )}
                        />
                        <FormField
                            control={form.control}
                            name="interestingFields"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>Interesting Fields</FormLabel>
                                    <FormControl>
                                        <MultiSelect
                                            options={interestingFields.map((field) => ({
                                                value: field.id,
                                                label: field.name,
                                                icon: field.icon,
                                            }))}
                                            onValueChange={(values) => field.onChange(values)}
                                            value={field.value || []}
                                            placeholder="Select your interesting fields"
                                            variant="inverted"
                                            animation={2}
                                            maxCount={3}
                                        />
                                    </FormControl>
                                    <FormDescription>Select at least 1 field</FormDescription>
                                    <FormMessage />
                                </FormItem>
                            )}
                        />
                        <FormField
                            control={form.control}
                            name="availableDays"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>Available Days</FormLabel>
                                    <FormControl>
                                        <MultiSelect
                                            options={availableDays.map((field) => ({
                                                value: field.id,
                                                label: field.name,
                                                icon: field.icon,
                                            }))}
                                            onValueChange={(values) => field.onChange(values)}
                                            value={field.value || []}
                                            placeholder="Your available days"
                                            variant="inverted"
                                            animation={2}
                                            maxCount={7}
                                        />
                                    </FormControl>
                                    <FormDescription>Select at least 1 field</FormDescription>
                                    <FormMessage />
                                </FormItem>
                            )}
                        />
                        <Button
                            type="submit"
                            disabled={isLoading}
                            variant={"default"}
                            className="w-full"
                        >
                            {isLoading ? (
                                <>
                                    <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                                    Creating account...
                                </>
                            ) : (
                                "Create account"
                            )}
                        </Button>
                    </form>
                </Form>
            </CardContent>
        </Card>
    );
    
}
