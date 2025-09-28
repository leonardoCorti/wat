// REST_api.js
const express = require("express");
const fs = require("fs");
const bodyParser = require("body-parser");
const { Client, LocalAuth, MessageMedia } = require("whatsapp-web.js");
const qrcode = require("qrcode-terminal");

const app = express();
const port = 3000;

const API_TOKEN = fs.readFileSync("api.token", "utf-8").trim();

function authenticate(req, res, next) {
  const authHeader = req.headers["authorization"];
  const token = authHeader && authHeader.split(" ")[1]; // "Bearer TOKEN"

  if (!token || token !== API_TOKEN) {
    return res.status(403).json({ error: "Forbidden" });
  }
  next();
}

app.use(bodyParser.json({ limit: "50mb" }));

// Init WhatsApp client
const client = new Client({
  authStrategy: new LocalAuth(),
  puppeteer: { headless: true }
});

// QR code in terminal
client.on("qr", (qr) => {
  qrcode.generate(qr, { small: true });
});

client.on("ready", () => {
  console.log("WhatsApp client is ready!");
});

// ========== REST API ROUTES ==========

app.get("/messages/:id", authenticate, async (req, res) => {
  try {
    const id = req.params.id;
    // id can be number (contact) or groupId
    const chatId = id.includes("@") ? id : `${id}@c.us`;

    const limit = req.query.limit ? parseInt(req.query.limit) : 10;

    const chat = await client.getChatById(chatId);
    const messages = await chat.fetchMessages({ limit });

    res.json(messages.map(m => ({
      from: m.from,
      body: m.body,
      timestamp: m.timestamp,
      hasmedia: m.hasMedia,
      id: m.id.id,
      isGroupMsg: m.isGroupMsg
    })));
  } catch (err) {
    console.error("Error fetching messages:", err);
    res.status(500).json({ error: "Could not fetch messages" });
  }
});

app.post("/send-text", authenticate, async (req, res) => {
  try {
    const { number, message } = req.body;
    if (!number || !message) return res.status(400).json({ error: "number and message required" });

    const chatId = number.includes("@") ? number : `${number}@c.us`;
    await client.sendMessage(chatId, message);

    res.json({ success: true, to: chatId, message });
  } catch (err) {
    console.error("Error sending text:", err);
    res.status(500).json({ error: "Could not send text message" });
  }
});

app.post("/send-image", authenticate, async (req, res) => {
  try {
    const { number, base64, caption } = req.body;
    if (!number || !base64) return res.status(400).json({ error: "number and base64 required" });

    const chatId = number.includes("@") ? number : `${number}@c.us`;
    const media = new MessageMedia("image/jpeg", base64);

    await client.sendMessage(chatId, media, { caption: caption || "" });
    res.json({ success: true, to: chatId, caption });
  } catch (err) {
    console.error("Error sending image:", err);
    res.status(500).json({ error: "Could not send image" });
  }
});

app.get("/chats", authenticate, async (req, res) => {
  try {
    const chats = await client.getChats();
    res.json(chats.map(c => ({
      id: c.id._serialized,
      name: c.name || null,
      isGroup: c.isGroup,
      lastMessage: c.lastMessage?.body || null
    })));
  } catch (err) {
    console.error("Error fetching chats:", err);
    res.status(500).json({ error: "Could not fetch chats" });
  }
});

//TODO /send-audio app.
//TODO /send-video app.
//TODO /send-file app.
//TODO /get-attachment app.

// Start API
client.initialize().then(() => {
  app.listen(port, "0.0.0.0", () => {
    console.log(`REST API running on http://localhost:${port}`);
  });
});

