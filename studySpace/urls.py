from django.urls import path
from .views import StudySpaceListView

urlpatterns = [
    path('', StudySpaceListView.as_view(), name='studySpace'),  
]