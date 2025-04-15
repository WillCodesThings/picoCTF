# 🧪 hash-crack-chain

## 🔍 Observations

- Connected to a remote service using netcat at `verbal-sleep.picoctf.net` on port `61522`.
- The service presents a sequence of hashes and asks for the plaintext password that matches each one.
- Initially presented with:
  ```
  482c811da5d5b4bc6d497ffa98491e38
  ```
  — which is an **MD5** hash.
- After correctly solving it, additional hashes of increasing complexity are revealed: **SHA-1**, then **SHA-256**.

---

## 🧠 Thought Process

- Ran Hashcat with mode `0` (MD5) and the **rockyou.txt** wordlist to identify the plaintext.
  ```
  482c811da5d5b4bc6d497ffa98491e38 -> password123
  ```
- After entering `password123`, a new SHA-1 hash appeared:
  ```
  b7a875fc1ea228b9061041b7cec4bd3c52ab3ce3
  ```
- Repeated the process with Hashcat mode `100` (SHA-1), same wordlist:
  ```
  b7a875fc1ea228b9061041b7cec4bd3c52ab3ce3 -> letmein
  ```
- Once that was cracked, a final SHA-256 hash was revealed:
  ```
  916e8c4f79b25028c9e467f1eb8eee6d6bbdff965f9928310ad30a8d88697745
  ```
- Used Hashcat mode `1400` (SHA-256) to find:
  ```
  916e8c4f79b25028c9e467f1eb8eee6d6bbdff965f9928310ad30a8d88697745 -> qwerty098
  ```

---

## 🧨 Exploitation

- Used Hashcat with these modes and commands:

  **MD5:**

  ```bash
  hashcat -m 0 482c811da5d5b4bc6d497ffa98491e38 /usr/share/wordlists/rockyou.txt
  ```

  **SHA-1:**

  ```bash
  hashcat -m 100 b7a875fc1ea228b9061041b7cec4bd3c52ab3ce3 /usr/share/wordlists/rockyou.txt
  ```

  **SHA-256:**

  ```bash
  hashcat -m 1400 916e8c4f79b25028c9e467f1eb8eee6d6bbdff965f9928310ad30a8d88697745 /usr/share/wordlists/rockyou.txt
  ```

- Successfully submitted each cracked password through the netcat session.

- Upon final success, the service revealed the flag:
  ```
  picoCTF{UseStr0nG_h@shEs_&PaSswDs!_36a1cf73}
  ```

---

## 📝 What I Learned

- Password hash cracking is easiest with weak/common passwords and rainbow tables like `rockyou.txt`.
- Knowing which hash algorithm you're facing is crucial — use hash length, tools like `hashid`, and context clues.
- Hashcat is an essential tool for CTFs: fast, flexible, and effective when paired with a good wordlist.
- Services that rely on simple hash validation without rate limiting or encryption are vulnerable.
- Always use **strong hashing algorithms** with **salting** and **rate limiting** to protect passwords.
