/**
 * BoxBtn Component
 * 
 * @param {Object} props - Component properties
 * @param {string} props.name - The name to display and use for the image source
 * @param {function} props.onClick - The click handler function
 */
export default ({name, onClick}) => {
    let hasDot = name.indexOf(".") != -1;
    return <div className={`box-btn ${hasDot?"mini": ""}`} onClick={onClick}>
        {
            hasDot?    
                <></>:<img src={`../${name}.svg`} />
        }
        <p>{name}</p>
    </div>
}