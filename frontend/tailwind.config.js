module.exports = {
    content: ["./index.html", "./src/**/*.rs"],
    theme: {
        extend: {
            colors: {
                'card-container-gray': '#151826',
                'primary-gray-1': '#282D42',
                'primary-gray-2': '#353A57',
                'secondary-gray': '#353A57',
                'secondary-gray-2': '#4D5481'
            }
        },
        fontFamily: {
            kanit: ['Kanit', 'sans-serif'],
        },
    },
    plugins: [
        // ...
        require('@tailwindcss/forms'),
    ],
};