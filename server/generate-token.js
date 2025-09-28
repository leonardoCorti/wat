const { randomBytes } = require("crypto");
const fs = require("fs");

const token = randomBytes(32).toString("hex"); // 64-char token
fs.writeFileSync("api.token", token);

console.log("Token generated:", token);

