import React, { FunctionComponent, ReactElement, ReactNode } from 'react';

import TopMenu from '../../menus/topMenu';

interface IBasicLayoutProps {
    /** The children to render. */
    children: ReactNode;
}

/**
 * Basic layout that is rendered around it's child components.
 *
 * @param {IBasicLayoutProps} props The properties of the basic layout.
 * @returns {ReactElement} The basic layout component.
 */
const BasicLayout: FunctionComponent<IBasicLayoutProps> = (props): ReactElement => {
    return (
        <div className="flex flex-1 flex-col max-h-screen">
            <TopMenu />
            <div className="flex flex-1 flex-col bg-gray-50">{props.children}</div>
        </div>
    );
};

export default BasicLayout;
