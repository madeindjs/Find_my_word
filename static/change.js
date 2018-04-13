let input = document.getElementById('word');
let list = document.getElementById('results');

let callback = (response) => {
	while (list.firstChild) list.removeChild(list.firstChild);

	let data = JSON.parse(response);
	data.results.forEach((result) => {
		let element = document.createElement('li');
		element.appendChild(document.createTextNode(result));
		list.appendChild(element);
	});
};

input.addEventListener('input',(e) => {
	// compatible with IE7+, Firefox, Chrome, Opera, Safari
	var xmlhttp;
	xmlhttp = new XMLHttpRequest();
	xmlhttp.onreadystatechange = function(){
	    if (xmlhttp.readyState == 4 && xmlhttp.status == 200){
	        callback(xmlhttp.responseText);
	    }
	}

	xmlhttp.open("GET", "/query/" + input.value, true);
	xmlhttp.send();
});



