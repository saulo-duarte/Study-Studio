from django.shortcuts import render
from django.views.generic import TemplateView

class DashboardView(TemplateView):
    template_name = 'dashboard/metrics.html'  # Verifique se este template existe

