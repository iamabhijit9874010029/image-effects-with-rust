function init() {
    const input = document.getElementById('upload');
    const filereader = new FileReader();

    filereader.onloadend = () => {
        // filereader.result holds the file contents after the file has been read using FileReader.readAsDataURL().
        let base64 = filereader.result.replace(/^data:image\/(png|jpeg|jpg);base64,/, '');
        console.log(input.files[0]);
        console.log(base64);

    }

    input.addEventListener('change', () => {
        filereader.readAsDataURL(input.files[0]);
    })
}

init();