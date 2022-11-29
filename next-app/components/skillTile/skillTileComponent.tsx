import React, { FunctionComponent, ReactElement } from 'react';

/**
 * Interface for the properties of the skill tile component.
 */
interface ISkillTileProps {
    /** The title of the tile. */
    title: string;
    /** The icon component to use on the tile. */
    icon: ReactElement;
    /** The hex code of the main color to use for the tile. */
    tileColor: string;
}

/**
 * Display component for one skill.
 *
 * @param {ISkillTileProps} props The properties of the skill tile.
 * @returns {ReactElement} The rendered skill component.
 */
const SkillTile: FunctionComponent<ISkillTileProps> = (props): ReactElement => {
    return (
        <div style={{ backgroundColor: props.tileColor }} className="flex flex-col justify-center items-center rounded-lg w-44 p-7 shadow">
            <div className="flex justify-center bg-white rounded-full w-16 h-16 p-2">
                <div className="flex justify-center items-center max-w-max max-h-full">{props.icon}</div>
            </div>
            <div className="flex justify-center mt-3 text-base font-bold rounded-full bg-white text-black px-3 py-1">{props.title}</div>
        </div>
    );
};

export default SkillTile;
