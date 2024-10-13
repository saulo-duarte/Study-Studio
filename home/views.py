# views.py
from django.shortcuts import render, redirect
from django.contrib.auth import authenticate, login
from django.contrib.auth.models import User
from django.contrib import messages

def home_view(request):
    return render(request, 'home/index.html')

def register(request):
    if request.method == 'POST':
        username = request.POST.get('username')
        password = request.POST.get('password')

        user = User.objects.create_user(username=username, password=password)
        user.save()
        messages.success(request, 'Account created successfully! You can now log in.')
        return redirect('login')

    return render(request, 'home/register.html')

def login_view(request):
    if request.method == 'POST':
        username = request.POST.get('username')
        password = request.POST.get('password')

        # Usuário hardcoded
        if username == 'saulo' and password == 'saulo':
            return redirect('home')  # Redireciona para a página inicial ou outra página após o login
        else:
            error_message = "Invalid username or password."
            return render(request, 'home/login.html', {'error_message': error_message})

    return render(request, 'home/login.html')