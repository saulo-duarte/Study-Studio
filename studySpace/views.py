from django.shortcuts import render
from django.views.generic import ListView
from .models import StudySpace
from django.contrib.auth.decorators import login_required




class StudySpaceListView(ListView):

    model = StudySpace
    template_name = 'studySpace/templates/index.html'
    context_object_name = 'studySpace'


@login_required
def studySpace_detail(request, pk):
    studySpace = StudySpace.objects.get(pk=pk)
    return render(request, 'studySpace/studySpace_detail.html', {'studySpace': studySpace})