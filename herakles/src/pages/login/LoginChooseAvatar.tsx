import React, { useRef, useEffect, useState } from 'react';
import { Card, CardHeader, CardFooter, Image, avatar } from '@nextui-org/react';
import SimpleBar from 'simplebar-react';
import 'simplebar-react/dist/simplebar.min.css';

const folders = {
    spring: 50,
    jobs: 50,
    sport: 30,
    game: 25
  };
  
const background_avatar_list = ["#120a1a","#2a2a41","#404863","#59607a","#787d8b","#9da5ae","#c8d6ac","#feffe5","#e4cd5a","#d49733","#d68552","#be5a1e","#894835","#602631","#4b0c30","#81173f","#cc1825","#dc4926","#f1934c","#fad5af","#ed9d7c","#d16363","#b7ab76","#b59857","#926d3c","#8b5b37","#ff82a0","#ff26a8","#422490","#2749d0","#4477ff","#4cc5e4","#8bf5c6","#85c448","#439d40","#29694e"]

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





const LoginChooseAvatar = ({ }) => {
  return (
    <Card
      radius="lg"
      className="border-none m-3"
    >
        <CardHeader className="flex gap-3 items-center justify-center">
            <p style={{fontSize:22}}>Choose avatar</p>
        </CardHeader>
        <SimpleBar data-simplebar-auto-hide="false" className='w-full flex flex-col items-start px-2'>
        <div className='flex flex-col justify-center'>
            {Object.entries(avatarImages).map(([categoryName, images]) => (
                <CategoryImages key={categoryName} categoryName={categoryName} images={images} />
            ))}
        </div>
        </SimpleBar>
    </Card>
  );
};

export default LoginChooseAvatar;
