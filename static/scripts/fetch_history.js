
function generate_table_header() {
  let header = document.createElement("tr");
  let name = document.createElement("th");
  let asset = document.createElement("th");
  let shares = document.createElement("th");
  let value = document.createElement("th");

  name.innerHTML = "Account Name";
  asset.innerHTML = "Asset";
  shares.innerHTML = "Shares";
  value.innerHTML = "Current Value (USD)";

  [name, asset, shares, value].forEach(title => {
    header.appendChild(title);
  });

  return header;
}

function create_record_element(record) {
  let head = document.createElement("div");
  let time = document.createElement("h3");
  let table = document.createElement("table");
  let net_worth_element = document.createElement("span");

  let net_worth = 0;

  table.appendChild(generate_table_header());

  time.innerHTML = record.time;
  record.holdings.forEach(holding => {
    let table_row = document.createElement("tr");
    // let holder = document.createElement("div");

    let name = document.createElement("td");
    let asset = document.createElement("td");
    let shares = document.createElement("td");
    let value = document.createElement("td");

    let worth = parseFloat(holding.shares) * record.prices[holding.asset];
    net_worth += worth;

    name.innerHTML = holding.name;
    asset.innerHTML = holding.asset;
    shares.innerHTML = holding.shares;
    value.innerHTML = String(worth);

    table_row.appendChild(name);
    table_row.appendChild(asset);
    table_row.appendChild(shares);
    table_row.appendChild(value);

    table.appendChild(table_row);
  });

  net_worth_element.innerHTML = "Net Worth: $" + String(net_worth);

  head.appendChild(time);
  head.appendChild(table);
  head.appendChild(net_worth_element);

  return head;
}

async function fetch_history() {
  return await fetch("/history").then((resp) => resp.json());
}

async function write_to_dom() {
  let output = document.getElementById("json_out");
  let records = await fetch_history();

  console.log(records);

  records.forEach((record) => {
    output.appendChild(create_record_element(record));
    output.appendChild(document.createElement("hr"));
  });
}

