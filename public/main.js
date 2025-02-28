async function init() {
    let rustApp = null;

    try {
        rustApp = await import('../pkg');
        console.log(rustApp);
    }
    catch (err) {
        console.error(err);
        return;
    }

    const input = document.getElementById('upload');
    const fileReader = new FileReader();

    fileReader.onloadend = () => {
        // fileReader.result holds the file contents after the file has been read using FileReader.readAsDataURL().
        let base64 = fileReader.result.replace(/^data:image\/(png|jpeg|jpg);base64,/, '');
        console.log(input.files[0]);
        console.log(base64);

    }

    input.addEventListener('change', () => {
        fileReader.readAsDataURL(input.files[0]);
    })
}

init();