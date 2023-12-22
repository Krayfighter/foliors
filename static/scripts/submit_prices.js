
function submit_prices() {
  let form = document.getElementById("prices_form");

  let assets = form.innerText.split('\n');

  let inputs = Array.from(form);
  let outputs = [];

  inputs.forEach(element => {
    outputs.push(parseFloat(element.value));
  });

  let final = assets.map((value, iter) => [value, outputs[iter]]);

  fetch(
    "/register_prices",
    {
      method: "POST",
      body: JSON.stringify(final),
      headers: {
        "Content-type": "application/json; charset=UTF-8"
      }
    }
  );
  // console.log(Array.from(form));

}


