setInterval(() => {
    const meteor = document.querySelector('.meteor');
    meteor.style.opacity = '1'; // Torna o meteoro visível
    meteor.style.animation = 'meteor-move 2s linear forwards'; // Inicia a animação

    meteor.addEventListener('animationend', () => {
        meteor.style.animation = ''; 
        meteor.style.opacity = '0';  
    });
}, 25000)
