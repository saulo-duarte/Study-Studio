'use client'

import { useState } from "react"
import { useRouter } from "next/navigation"
import { zodResolver } from "@hookform/resolvers/zod"
import { useForm } from "react-hook-form"
import * as z from "zod"

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
    CardFooter } from "./ui/card"


import { useToast } from "@/hooks/use-toast"
import { Loader2 } from "lucide-react"

const formSchema = z.object({
    name: z.string()
    .min(2, "Name must be at least 2 characters")
    .max(30, "Name must be at most 30 characters")
    .nonempty("Name is required"),
    email: z.string()
    .email("Invalid email address")
    .nonempty("Email is required"),

    preferences: z.object({
        theme: z.enum(["light", "dark"]),
    }).optional(),

})

type FormValues = z.infer<typeof formSchema>

export function OnboardingForm(){
    const router = useRouter()
    const { toast } = useToast()
    const [isLoading, setisLoading] = useState(false)

    const form = useForm<FormValues>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            name: "",
            email: "",
            preferences: {
                theme: "dark",
            },
        }
    })

    async function onSubmit(data: FormValues){
        try {
            setisLoading(true)
            
            await new Promise((resolve) => setTimeout(resolve, 1000))
            
            toast({
                title: "Account created",
                description: "Your account has been created successfully",
            })

            router.push("/")
        } catch (error) {
            toast({
                variant: "destructive",
                title: "An error occurred",
                description: "An error occurred while creating your account. Please try again",
            })
            
        } finally {
        setisLoading(false)
        }
    }

    return(
        <Card className="w-full max-w-md mx-auto">
            <CardHeader>
                <CardTitle> Welcome to Study Studio </CardTitle>
                <CardDescription> Tell a little bit about you </CardDescription>
            </CardHeader>
            <CardContent>
                <Form {...form}>
                    <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">
                        <FormField
                            control={form.control}
                            name="name"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel> Name </FormLabel>
                                    <FormControl>
                                        <Input
                                            placeholder="Your name"
                                            {...field}
                                            disabled={isLoading}
                                        />
                                    </FormControl>
                                    <FormDescription>
                                        How should we call you?    
                                    </FormDescription> 
                                    <FormMessage />
                                </FormItem>
                            )}
                            />

                        <FormField
                            control={form.control}
                            name="email"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel> Email </FormLabel>
                                    <FormControl>
                                        <Input
                                            placeholder="youremail@email.com"
                                            {...field}
                                            disabled={isLoading}
                                        />
                                        </FormControl>
                                        <FormDescription>
                                            We'll never share your email with anyone else.
                                            </FormDescription>
                                            <FormMessage />
                                            </FormItem>
                                            )}
                        />
                    </form>
                </Form>
            </CardContent>
            <CardFooter className="flex justify-end gap-4">
                <Button
                    type="submit"
                    onClick={form.handleSubmit(onSubmit)}
                    disabled={isLoading}
                >
                    {isLoading ? (
                        <>
                            <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                            Creating account...
                        </>
                    ): (
                        "Create account"
                    )}
                </Button>
            </CardFooter>
        </Card>
    )

}