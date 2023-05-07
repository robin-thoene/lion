module.exports = {
    mode: 'jit',
    content: ['./pages/**/*.{js,ts,jsx,tsx}', './components/**/*.{js,ts,jsx,tsx}'],
    darkMode: 'media',
    theme: {
        extend: {
            boxShadow: {
                DEFAULT: '0px 6px 15px 0 rgba(0, 0, 0, 0.2)',
            },
            colors: {
                black: '#333333',
            },
            fontSize: {
                xxs: '0.625rem',
            },
        },
    },
    variants: {
        extend: {},
    },
    daisyui: {
        styled: true,
        base: true,
        utils: true,
        logs: true,
        rtl: false,
        prefix: '',
        themes: ['light', 'dark'],
    },
    plugins: [require('daisyui')],
};
