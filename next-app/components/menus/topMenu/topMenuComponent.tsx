import { faGithub, faLinkedinIn, faXing } from '@fortawesome/free-brands-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import React, { FunctionComponent, ReactElement } from 'react';

import DarkModeToggle from '../../darkModeToggle';
import LanguageSelector from '../../languageSelector';

/**
 * Top menu that contains routes and actions.
 *
 * @returns {ReactElement} The top menu component.
 */
const TopMenu: FunctionComponent = (): ReactElement => {
    return (
        <div className="fixed top-0 left-0 flex w-full py-4 px-8 items-center bg-base-100 xl:bg-transparent z-30">
            <a aria-label="GitHub" href="https://github.com/robin-thoene" target="_blank" rel="noreferrer">
                <FontAwesomeIcon icon={faGithub} size="lg" className="h-5 w-5" />
            </a>
            <a aria-label="LinkedIn" className="ml-4" href="https://linkedin.com/in/robin-thöne-681870205" target="_blank" rel="noreferrer">
                <FontAwesomeIcon icon={faLinkedinIn} size="lg" className="h-5 w-5" />
            </a>
            <a aria-label="XING" className="ml-4" href="https://www.xing.com/profile/Robin_Thoene" target="_blank" rel="noreferrer">
                <FontAwesomeIcon icon={faXing} size="lg" className="h-5 w-5" />
            </a>
            <div className="flex ml-auto">
                <DarkModeToggle />
                <LanguageSelector />
            </div>
        </div>
    );
};

export default TopMenu;
