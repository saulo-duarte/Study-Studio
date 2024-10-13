from django.shortcuts import render
from django.views.generic import ListView
from .models import Topic
# Create your views here.
class TopicsListView(ListView):
    model = Topic
    template_name = 'topics/topics_list.html'