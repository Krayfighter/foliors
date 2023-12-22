

let accounts = []

async function create_chart_options_form() {
  
  accounts = await fetch(
    "/accounts"
  ).then((resp) => resp.json());

  console.log(accounts);

  let form = document.getElementById("options");

  accounts.forEach(account => {
    let input = document.createElement("input");
    input.setAttribute("type", "checkbox");
    input.setAttribute("id", account);

    let label = document.createElement("label");
    label.setAttribute("for", account);
    label.innerHTML = account;

    form.appendChild(input);
    form.appendChild(label);
  });

  let holder = document.getElementById("holder");

  let submit_button = document.createElement("button");
  submit_button.setAttribute("onclick", "submit_chart_options()");
  submit_button.innerHTML = "render chart";

  let enable_all_button = document.createElement("button");
  enable_all_button.setAttribute("onclick", "enable_all()");
  enable_all_button.innerHTML = "enable all";

  holder.appendChild(submit_button);
  holder.appendChild(enable_all_button);

}

function enable_all() {
  accounts.forEach(account => {
    document.getElementById(account).checked = true;
  });
}

async function submit_chart_options() {
  let checked_accounts = [];
  accounts.forEach(account => {
    let element = document.getElementById(account);
    if (element.checked) { checked_accounts.push(account); }
  });
  let options = {
    accounts: checked_accounts
  };

  await fetch(
    "/make_chart",
    {
      method: "POST",
      body: JSON.stringify(options),
      headers: {
        "Content-type": "application/json; charset=UTF-8"
      }
    }
  );

  location.reload();
}



