let element = document.getElementById('word');

let callback = (response) => {
	console.log(response);
};

element.addEventListener('input',(e) => {
	// compatible with IE7+, Firefox, Chrome, Opera, Safari
	var xmlhttp;
	xmlhttp = new XMLHttpRequest();
	xmlhttp.onreadystatechange = function(){
	    if (xmlhttp.readyState == 4 && xmlhttp.status == 200){
	        callback(xmlhttp.responseText);
	    }
	}

	xmlhttp.open("GET", "/query/" + element.value, true);
	xmlhttp.send();
});



