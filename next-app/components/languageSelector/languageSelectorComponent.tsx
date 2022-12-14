import { LanguageIcon } from '@heroicons/react/24/solid';
import Link from 'next/link';
import { useRouter } from 'next/router';
import { useTranslation } from 'next-i18next';
import React, { FunctionComponent, ReactElement } from 'react';

/**
 * The icon to use for the language selection opening button.
 */
const icon = <LanguageIcon className="h-5 w-5" />;

/**
 * Component to select and switch the application display language.
 *
 * @returns {ReactElement} The language selector component.
 */
const LanguageSelector: FunctionComponent = (): ReactElement => {
    /** Access to translations. */
    const { t, i18n } = useTranslation();
    /** Access to the next js router. */
    const router = useRouter();

    return (
        <div className="dropdown dropdown-end">
            <label aria-label={t('Language_Button_Aria_Label')} tabIndex={0} className="btn btn-circle btn-ghost h-12 w-12 animate-none">
                {icon}
            </label>
            <ul tabIndex={0} className="dropdown-content bg-white dark:bg-base-100 menu shadow rounded-box w-40">
                <li className={`${i18n.language === 'en' && 'disabled'}`}>
                    {i18n.language !== 'en' ? (
                        <Link aria-label={t('Language_Button_En_Aria_Label')} className="p-3 flex justify-center" href={router.asPath} locale={'en'}>
                            {t('Language_Option_En')}
                        </Link>
                    ) : (
                        <div className="p-3 flex justify-center">{t('Language_Option_En')}</div>
                    )}
                </li>
                <li className={`${i18n.language === 'de' && 'disabled'}`}>
                    {i18n.language !== 'de' ? (
                        <Link aria-label={t('Language_Button_De_Aria_Label')} className="p-3 flex justify-center" href={router.asPath} locale={'de'}>
                            {t('Language_Option_De')}
                        </Link>
                    ) : (
                        <div className="p-3 flex justify-center">{t('Language_Option_De')}</div>
                    )}
                </li>
            </ul>
        </div>
    );
};

export default LanguageSelector;
