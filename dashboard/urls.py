from django.urls import path
from . import views

urlpatterns = [
    path('', views.DashboardView.as_view(), name='dashboard'),  # Ajuste conforme sua view
    # Adicione outras URLs conforme necessário
]
