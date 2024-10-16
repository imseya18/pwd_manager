import React, { useRef, useEffect, useState } from 'react';



const LoginCategoryImages = ({ categoryName, images, backgroundAvatar, selectedImage, onSelectImage }) => {
    const parentRef = useRef(null);
    const [imagesPerRow, setImagesPerRow] = useState(0);

    useEffect(() => {
      const calculateImagesPerRow = () => {
        if (parentRef.current) {
          const parentWidth = parentRef.current.offsetWidth;
          
          

          const imageWidth = 115; // Largeur fixe de l'image
          const margin = 20;
          const imageTotalWidth = 135;

          //console.log(window.innerWidth, Math.floor(window.innerWidth * 0.8 / imageTotalWidth) || 1);
  
          // Calculer le nombre maximal d'images par ligne
          let maxImagesPerRow = Math.floor(parentWidth / imageTotalWidth) || 1;
          if (maxImagesPerRow > 8)
            maxImagesPerRow = 8;
          
          setImagesPerRow(maxImagesPerRow);
        }
      };
  
      calculateImagesPerRow();
  
      // Recalculer lors du redimensionnement de la fenêtre
      window.addEventListener('resize', calculateImagesPerRow);
  
      // Nettoyage
      return () => {
        window.removeEventListener('resize', calculateImagesPerRow);
      };
    }, []);
  
    // Calculer la largeur totale du div flex-wrap
    const imageTotalWidth = 135;
  
    const flexWrapWidth = imagesPerRow * imageTotalWidth;
  
    return (
      <div
        ref={parentRef}
        className="flex items-center justify-center"
        style={{ background: 'transparent'}}
      >
        <div
          className="flex flex-wrap"
          style={{ width: `${flexWrapWidth}px` ,  background: 'transparent'}}
        >
          {images.map((imgSrc, index) => (
            <img
              key={imgSrc}
              src={imgSrc}
              alt={`${categoryName} avatar`}
              width={115}
              className="avatar-img-selection"
              style={{
                margin: 10, 
                paddingTop: 5, 
                background: backgroundAvatar,
                border: selectedImage === imgSrc ? '2px solid white' : 'none', // Appliquer le bord si sélectionné
                cursor: 'pointer'
              }}
              onClick={() => onSelectImage(imgSrc)}
            />
          ))}
        </div>
      </div>
    );
  };
  
  export default LoginCategoryImages;
  