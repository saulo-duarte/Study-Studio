# topics/urls.py
from django.urls import path
from . import views

urlpatterns = [
    path('', views.TopicsListView.as_view(), name='topics'),  # Certifique-se de que o nome da view é 'topics'
]
