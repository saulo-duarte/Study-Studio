from django.shortcuts import render
from django.views.generic import ListView
from .models import Task  # Assegure-se de que o modelo Task exista

class TaskListView(ListView):
    model = Task
    template_name = 'tasks/tasks_list.html'  # Verifique se este template existe
    context_object_name = 'tasks'  # O nome da variável que será utilizada no template
