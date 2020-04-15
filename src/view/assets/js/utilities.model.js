class Utilities {
    static uuidv4() {
        //credit: https://stackoverflow.com/questions/105034/create-guid-uuid-in-javascript
        return ([1e7]+-1e3+-4e3+-8e3+-1e11).replace(/[018]/g, c =>
            (c ^ crypto.getRandomValues(new Uint8Array(1))[0] & 15 >> c / 4).toString(16)
        )
    }
    static random_characters(length) {
        let characters = '0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz';
        // shuffle all characters to remove sequence as a predicable variable
        let shuffled = '';
        for (let i = 0; i<characters.length; i++) shuffled += characters.charAt(Utilities.secure_random(characters.length));
        // create returned string
        let value = '';
        for (let i = 0; i<length; i++) {
            let position = Utilities.secure_random(shuffled.length);
            value += shuffled.charAt(position);
        }

        return value;
    }
    static secure_random(max) {
        //credit: https://developer.mozilla.org/en-US/docs/Web/API/Crypto/getRandomValues
        return Math.floor(crypto.getRandomValues(new Uint32Array(1))[0] / ((2^32) - 1) % max);
    }
    static HTMLEncode(string) {
        let element = document.createElement("div");
        element.innerText = element.textContent = string;
        return element.innerHTML;
    }
    static addClipboard(string) {
        if (!navigator.clipboard) {
            let textArea = document.createElement('textarea');
            textArea.value = string;
            textArea.style.position = "fixed";
            document.body.append(textArea);
            textArea.focus();
            textArea.select();

            try {
                 document.execCommand("copy");
                 return true;
            } catch (err) {
                console.error("WARNING: Automated copying was unsuccessful reason: ", err);
                return false;
            }
        } else {
            navigator.clipboard.writeText(string).then(function() {
                return true;
            }, function(err) {
                console.error("WARNING: Automated copying was unsuccessful reason: ", err);
                return false;
            })
        }
    }
    static isValidURL(string) {
        let pattern = new RegExp('^(https?:\\/\\/)?'+ // protocol
            '((([a-z\\d]([a-z\\d-]*[a-z\\d])*)\\.)+[a-z]{2,}|'+ // domain name
            '((\\d{1,3}\\.){3}\\d{1,3}))'+ // OR ip (v4) address
            '(\\:\\d+)?(\\/[-a-z\\d%_.~+]*)*'+ // port and path
            '(\\?[;&a-z\\d%_.~+=-]*)?'+ // query string
            '(\\#[-a-z\\d_]*)?$','i'); // fragment locator
        return !!pattern.test(string);
    }
}