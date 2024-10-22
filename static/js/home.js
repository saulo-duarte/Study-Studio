setInterval(() => {
    const meteor = document.querySelector('.meteor');
    meteor.style.opacity = '1'; // Torna o meteoro visível
    meteor.style.animation = 'meteor-move 2s linear forwards'; // Inicia a animação

    meteor.addEventListener('animationend', () => {
        meteor.style.animation = ''; 
        meteor.style.opacity = '0';  
    });
}, 25000)

document.addEventListener("DOMContentLoaded", function() {

    const cards = document.querySelectorAll('.tasks-cards div');

    cards.forEach(function(card) {

        const title = card.querySelector('h2').textContent;

        if (title == 'Review') {
            card.style.border = '4px solid purple';
            card.querySelector('h2').style.color = 'purple';
        
        } 
        
        else if (title == 'Study') {
            card.style.border = '4px solid orange';
            card.querySelector('h2').style.color = 'orange';
        }

        else {
            card.style.border = '4px solid green';
            card.querySelector('h2').style.color = 'green';
        }
    });

});