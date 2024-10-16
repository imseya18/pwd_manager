import React, { useRef, useEffect, useState } from 'react';
import { Card, CardHeader, CardFooter, Image, avatar, Button } from '@nextui-org/react';
import SimpleBar from 'simplebar-react';
import 'simplebar-react/dist/simplebar.min.css';

import LoginCategoryImages from "./LoginCategoryImages"

const folders = {
    spring: 50,
    jobs: 50,
    sport: 30,
    game: 25
  };
  
  const background_avatar_list = ["#2a2a41", "#404863", "#008148", "#EF8A17", "#cc1825","#be5a1e","#894835","#602631","#4b0c30","#d16363","#ff26a8","#926d3c","#8b5b37","#29694e"];

  const basePath = 'src/media/img/avatar';
  const avatarImages = {};
  
  
  Object.entries(folders).forEach(([folderName, numberOfImages]) => {
    avatarImages[folderName] = [];
    for (let image = 1; image <= numberOfImages; image++) {
      avatarImages[folderName].push(`${basePath}/${folderName}/${image}.svg`);
    }
  });

  const CategoryImages = ({ categoryName, images }) => (
    <div className='flex items-center justify-center' style={{background:'red'}}>
      <div className="flex flex-wrap">
        {images.map(imgSrc => (
          <img key={imgSrc} src={imgSrc} alt={`${categoryName} avatar`} width={115} className='m-5 avatar-img-selection' />
        ))}
      </div>
    </div>
  );

  const AvatarColors = ({ onSelectColor }) => {
    const [selectedColor, setSelectedColor] = useState(background_avatar_list[0]); // Default to the first color
  
    return (
      <div className='w-full items-center flex flex-row flex-wrap'>
        {background_avatar_list.map((color, index) => (
          <div key={index} 
            style={{
              backgroundColor: color, 
              padding: '10px', 
              margin: '5px', 
              borderRadius: 15, 
              cursor: 'pointer',
              border: selectedColor === color ? '2px solid white' : 'none' // Apply border if selected
            }}
            onClick={() => {
              onSelectColor(color);
              setSelectedColor(color);
            }}
          />
        ))}
      </div>
    );
  };
  





const LoginChooseAvatar = ({ }) => {
  const [selectedColor, setSelectedColor] = useState(background_avatar_list[0]);
  const [selectedImage, setSelectedImage] = useState(null);

  return (
    
      <Card
        radius="lg"
        className="border-none m-3"
        style={{maxWidth:1200, margin: "0 auto"}}
      >
          <CardHeader className="flex flex-col   gap-3 items-center justify-center">
          <p style={{fontSize:26, marginBottom:5, fontWeight:"bold"}}>Choose avatar</p>
              <div className='w-full items-center flex flex-row justify-center'>
            <p style={{fontWeight:"bold", marginRight: 10}}>Choose background:</p>
            <div style={{width:"70%", overflowX:"hidden"}}><AvatarColors onSelectColor={setSelectedColor}/></div>
          </div>
          </CardHeader>
          
          <SimpleBar data-simplebar-auto-hide="false" className='w-full flex flex-col items-start px-2' style={{ height: 600 }}>
          <div className='flex flex-col justify-center'>
              {Object.entries(avatarImages).map(([categoryName, images]) => (
                  <LoginCategoryImages
                    key={categoryName}
                    categoryName={categoryName}
                    images={images}
                    backgroundAvatar={selectedColor}
                    selectedImage={selectedImage}
                    onSelectImage={setSelectedImage}
                    />
              ))}
          </div>
          </SimpleBar>
          <CardFooter className='flex flex-row-reverse'>
            <Button style={{marginRight:55, paddingLeft:50, paddingRight:50}} color="primary"><p style={{fontWeight:"bold"}}>Save</p></Button>
          </CardFooter>
      </Card>

  );
};

export default LoginChooseAvatar;
