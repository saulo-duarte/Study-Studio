from django.db import models

# Create your models here.
class Topic(models.Model):
    title = models.CharField(max_length=200)  # ou outros campos que você precisa
    description = models.TextField()

    def __str__(self):
        return self.title