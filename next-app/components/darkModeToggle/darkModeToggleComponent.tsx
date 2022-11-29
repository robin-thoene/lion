import { MoonIcon, SunIcon } from '@heroicons/react/24/solid';
import { useTranslation } from 'next-i18next';
import React, { FunctionComponent, ReactElement, useEffect, useState } from 'react';

/**
 * Component to activate / de-activate the dark mode using a UI toggle.
 *
 * @returns {ReactElement} The dark mode toggle component.
 */
const DarkModeToggle: FunctionComponent = (): ReactElement => {
    /** Access to translations. */
    const { t } = useTranslation();

    /** Whether the dark mode is enabled or not. */
    const [darkModeEnabled, setIsDarkModeEnabled] = useState<boolean>(false);

    /** Keep track on system preferences. */
    useEffect(() => {
        // Check if a theme is set explicit.
        const htmlElement = document.documentElement;
        if (htmlElement.hasAttribute('data-theme')) {
            const theme = htmlElement.getAttribute('data-theme');
            setIsDarkModeEnabled(theme === 'dark');
            return;
        }
        const isDarkPreferred = window.matchMedia('(prefers-color-scheme: dark)').matches;
        setIsDarkModeEnabled(isDarkPreferred);
    }, []);

    /**
     * Handle switch between light and dark theme.
     *
     * @param {boolean} enableDarkMode Whether the dark mode shall be enabled or not.
     */
    const handleThemeSwitch = (enableDarkMode: boolean) => {
        const htmlElement = document.documentElement;
        if (enableDarkMode) {
            // Activate the dark theme.
            htmlElement.setAttribute('data-theme', 'dark');
        } else {
            // Activate the light theme.
            htmlElement.setAttribute('data-theme', 'light');
        }
        setIsDarkModeEnabled(enableDarkMode);
    };

    return (
        <label className="swap swap-rotate btn btn-circle btn-ghost mb-1 sm:h-12 sm:w-12 min-h-8 h-8 w-8 animate-none">
            <input
                aria-label={t('DarkModeToggle_Toggle_Aria_Label')}
                type="checkbox"
                checked={darkModeEnabled}
                onChange={(event) => {
                    const checked = event.target.checked;
                    handleThemeSwitch(checked);
                }}
            />
            <SunIcon className="swap-off fill-current h-3 w-3 sm:h-5 sm:w-5" />
            <MoonIcon className="swap-on fill-current current h-3 w-3 sm:h-5 sm:w-5" />
        </label>
    );
};

export default DarkModeToggle;
