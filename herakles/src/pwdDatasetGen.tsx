const pwdDatasetGen = () => {
  const sites = [
    'https://chatgpt.com',
    'https://dokca.com',
    'https://youtube.com',
    'https://reddit.com',
    'https://stackoverflow.com',
    'https://haeghjfgjherfhj.com'
  ];

  // Tableau de notes possibles
  const notesArray = [
    'Note importante',
    'À vérifier plus tard',
    'Compte secondaire',
    'Mot de passe changé récemment',
    'Ne pas partager',
    'Compte principal',
    'Mot de passe temporaire',
    'Vérifier les mises à jour',
    'Compte professionnel',
    'Compte personnel',
  ];

  const dataset = sites.map((url, index) => {
    const accountNumber = Math.floor(Math.random() * 5) + 1; // Génère un nombre entre 1 et 5
    const accounts = [];

    for (let i = 0; i < accountNumber; i++) {
      const randomNote = notesArray[Math.floor(Math.random() * notesArray.length)];
      accounts.push({
        email: `user${i + 1}@gmail.com`,
        password: `password${i + 1}`,
        note: randomNote,
        last_modif: Date.now() - Math.floor(Math.random() * 1000000000),
      });
    }

    return {
      id: (index + 1).toString(),
      url,
      account_number: accountNumber,
      accounts: accounts,
    };
  });

  return dataset;
};

export default pwdDatasetGen;
