# **Project Design Document: StegoDrop**

Subtitle: The Anti-Metadata PQC Utility

Version: 4.18 (Master MVP Specification)

Date: December 14, 2025

Platform: Native iOS (Swift) & Native Android (Kotlin)

Architecture: Local-First / Serverless / Transport-Agnostic

Compliance Category: Utilities / Productivity (NOT Social NETworking)

## **1\. Executive Summary**

The Concept:

StegoDrop is a cryptographic utility that allows users to hide Post-Quantum secure messages inside standard image files. To the naked eye, the image is just a photo. To the intended recipient, it contains secure communication.

The Problem:

Current secure messengers and communication platforms protect content but expose metadata (who talks to whom). Additionally, this metadata is vulnerable to "Harvest Now, Decrypt Later" attacks via future Quantum Computers.

The Solution:

A local-only, serverless tool that decouples identity from transport. By embedding encrypted payloads into innocuous media (social media posts, email attachments, forum uploads, etc.), StegoDrop renders the communication channel statistically invisible and Quantum-resistant.

**Core Philosophy:** "Hide in Plain Sight."

## **2\. Technical Architecture**

### **2.1 The "Local-Only" Model**

* **No Servers:** The app connects to NO backend APIs.  
* **No Accounts:** Identity is a locally generated cryptographic key pair.  
* **No Metadata:** Since we don't transport messages, we generate no metadata graphs.  
* **Networking:** Zero. The "Network" is the user's file system and sharing sheet.

### **2.2 Split-Layer Cryptography**

We utilize a **Split-Layer Architecture** to balance the heavy mathematical requirements of Post-Quantum Cryptography (PQC) with the aggressive compression constraints of social media.

* **Layer 1: The Handshake (Overt)**  
  * **Purpose:** Establish a shared secret key (Identity).  
  * **Transport:** High-reliability channels (QR Codes via AirDrop, Signal, or Physical Scan).  
  * **Algo:** **Kyber-768** (Post-Quantum KEM). Keys are too large (\~1KB) for robust steganography, so we use QR codes as the container.  
* **Layer 2: The Transport (Covert)**  
  * **Purpose:** Daily communication (Messaging).  
  * **Transport:** Low-bandwidth, covert channels (high-traffic public forums, social media, email).  
  * **Algo:** **AES-256-GCM** (Quantum-Resistant).  
  * **Stego:** **DCT Mid-Band Embedding**. Survives JPEG compression because the payload is tiny (\<200 bytes).

## **3\. Identity & Key Management**

### **3.1 Hierarchical Key Structure**

* **Root Key (The Crown):** Used *only* to sign/authorize new Delegate Keys. Never used for encryption.  
* **Delegate Key (The Hand):** Used to encrypt/decrypt daily messages. Lives in the device's Secure Enclave.  
* **Verification:** Users verify the *Root Key* during the QR Handshake. The App automatically trusts any *Delegate Key* signed by that Root.

### **3.2 Security Tiers ("Choose Your Mode")**

* **Standard Mode:** Root Key stored in Cloud Keychain (iCloud/Google) for easy migration.  
* **Ghost Mode:** \* Root Key exists *only* as a physical QR code (printed/saved offline). The device memory is wiped of the Root Key after generation.  
  * **UX Requirement:** Must provide the option to **skip local storage** of contact aliases (n), handles (h), and channel names (c) after handshake completion. The contact will be stored as a raw Public Key Hash instead.

### **3.3 The "Smart Burn" Protocol (Compromise Recovery)**

* **Trigger:** User initiates **"Panic Mode"** (requires **PIN \+ Bio-Auth**).  
* **Action:**  
  1. App generates a new Delegate Key.  
  2. App signs the new Delegate Key using the old Root Key (Trust Chain).  
  3. The old Delegate Key is revoked.  
* **Propagation:** The next message sent includes the new Delegate Public Key. Recipients' apps verify the Root Signature and automatically update the contact, locking out the old (revoked) key.

## **4\. Features & User Flow**

### **4.1 Onboarding: The QR Handshake**

1. **Generate Invite (User A):** User A inputs their preferred **Handle** (e.g., @user\_a). *Inputting the Alias/Handle must be optional.* App creates a **QR Code** containing the Kyber Public Key \+ Alias \+ Handle. **Optional:** User can toggle a central "Ghost" logo overlay. User A shares this image.  
2. **Accept & Reply (User B):** User B scans the QR. App encapsulates a Shared Secret and generates a **Response QR**. User B sends this back.  
3. **Validate (User A):** User A scans the Response QR. App displays a confirmation: "Handshake from  
4. $$User B Alias/Handle\] or \\\[Public Key Hash$$  
5. . Confirm?"  
6. **Result:** Secure link established.

### **4.2 Messaging: "Shadow Mode" (Steganography)**

1. **Compose:** User selects a cover photo (e.g., a sunset). User types message.  
2. **Process:**  
   * **EXIF Strip:** App must strip **all** EXIF and proprietary metadata (GPS, camera make, time stamp, etc.) from the image before embedding.  
   * **Embedding:** App encrypts text (AES-256) and embeds it into the **Mid-Frequency DCT coefficients** of the image.  
3. **Transport:** User utilizes **Native Share Sheet** to send the generated image (to Instagram, SMS, AirDrop, Printer, etc.).  
4. **Read:** User B receives the image, shares it to the **StegoDrop** app. App detects payload, decrypts, and displays message.

### **4.3 Backup: "Visible Mode"**

* **Scenario:** Image quality is too low for steganography (e.g., MMS).  
* **Action:** User toggles "Visible Mode."  
* **Output:** App generates a **QR Code** containing the encrypted message (t: 'm'). This sacrifices stealth for 100% reliability.  
  * **Optional:** Toggle logo overlay for visual flair.

### **4.4 Moderation: "Black Hole" Protocol**

* **Philosophy:** We are a Tool, not a Social Network. We do not host content.  
* **Block Feature:** Users can "Block" a Public Key. The app refuses to decrypt future messages from that sender, displaying a warning: "Message detected from blocked user  
* $$Alias/Handle or Public Key Hash$$  
* ."  
* **Report Feature:** A local-only action that deletes the content from the device and blocks the sender. No data is reported to a central server (as none exists).

## **5\. Security Specifications**

### **5.1 Anti-Forensics**

* **Screenshot Defense:** On recovery and decrypt screens, security flags will be applied.  
  * *Android:* Apply FLAG\_SECURE to prevent screen capture.  
  * *iOS:* Utilize secure views/context warnings to mitigate risk.  
* **Coercion Resistance:** There is **NO** "Show Private Key" button in Settings. The key is shown ONCE at creation, then the UI code path is removed.

### **5.2 Disaster Recovery**

* **Paper Key:** Ghost Mode recovery relies solely on a one-time paper key generated during setup.  
* **Format:** The recovery key is stored as a custom URI: stgo://restore/v1?k=\[Base64Blob\]  
* **Requirement:** User must print/write this key down. It is the ONLY way to recover a "Ghost Mode" account.

## **6\. Technical Specifications**

### **6.1 Payload Schema (Unified JSON)**

* {  
*   "v": 1,               // Protocol Version  
*   "t": "i",             // Type: 'i' (invite), 'r' (response), 'm' (message)  
*   "n": "UserAlias",     // Sender Display Name (Optional)  
*   "h": "@username",     // Contact Handle (Optional \- e.g. Instagram handle)  
*   "c": "ig",            // Reply Preference (Enum: 'ig', 'wa', 'signal')  
*   "d": "BASE64\_STRING"  // The Payload Data (Kyber Key, Ciphertext, or AES Blob)  
* }

### **6.2 Error Handling Strategy**

* **Reactive:** Allow users to try any transport method. Only intervene if the math fails.  
* **Scan Failures:** *"Image quality too low. Ask sender for a high-res file."*  
* **Stego Failures:** *"No hidden message found or image was scrubbed."*  
* **Replay/Integrity Failure:**  
  * **Rejection (Backward/Replay):** Any message with a counter **equal to or less than** the last read counter must be rejected. Display: *"Message rejected: The message you're trying to read is out-of-order."*  
  * **Warning (Skip):** Any message with a counter **greater than N+1** must be processed, but display a warning to the user: "Message gap detected:  
  * $$N$$  
  * messages missed or out of order."

### **6.3 QR Branding Specification**

* **Error Correction Level:** Must use **Level H (30%)** when Logo Toggle is ON.  
* **Mask Area:** Central 15-20% of the QR code area.  
* **Logo Asset:** Monochrome or High-Contrast vector of the App Icon (Ghost).  
* **Rationale:** Level H allows 30% of data to be lost (covered by logo) while still maintaining full scannability.

### **6.4 Anti-Detection (Steganalysis Resistance)**

* **Anti-Replay Counter:** The AES payload must include a **monotonic, strictly increasing message counter**. The recipient must reject any message where the counter violates the sequence (see 6.2).  
* **Randomized Selection:** The choice of DCT coefficients for embedding must be based on a **per-message pseudo-random seed** (derived from the AES key or Nonce). This prevents a unified statistical fingerprint for the entire application.  
* **Hybrid Scaling:** The amplitude of the modification applied to the coefficients should be **randomly scaled** ($\\pm 1$ or $\\pm 2$), rather than a fixed value, to mask the expected uniform shift in the DCT histogram that traditional steganalysis tools look for.

## **7\. Compliance & Legal Strategy**

### **7.1 Licensing**

* **License:** **MIT License**  
* **Rationale:** Provides maximum freedom to use, modify, and audit the code. This is essential for building trust and credibility within the cybersecurity community and retains all commercialization rights for future paid versions.

### **7.2 App Store Strategy**

* **Category:** "Utilities" or "Productivity."  
* **UGC Policy:** Use the "Black Hole" reporting method to satisfy Guideline 1.2 (User Generated Content) without incurring liability for hosting.  
* **Privacy Policy:** "We collect nothing."  
* **Iconography:** Transparent branding ("Privacy Utility"), avoiding "Fake Calculator" tropes that trigger rejection.

### **7.3 Export Control**

* **USA:** File Annual Self-Classification Report (mass market encryption) with BIS.  
* **Geofencing:** Exclude **China, Russia, France** (MVP) from App Store distribution to avoid complex encryption licensing requirements.

## **8\. Known Constraints & Development Roadmap**

### **8.1 Known Constraints**

* **Image Format:** Input image must be high-fidelity (e.g., PNG, or high-quality JPEG).  
* **Image Size:** Minimum input resolution **500x500px** is required to reliably hide the smallest payload (the AES message) and preserve the integrity of the QR codes.  
* **Format Caveat:** Steganography output is highly sensitive to external compression. Users must be discouraged from low-quality image exports.  
* **Message Sequencing:** Due to the strict key ratcheting and anti-replay defense, **messages can only be successfully decrypted once and must be read in order.** The user will be warned if a message is skipped and cannot retrieve the missed message later.

### **8.2 Development Roadmap**

**Phase 1: The Engine (The Math)**

* \[ \] Implement CryptoManager: Kyber-768 Key Gen \+ AES-256 pipeline.  
* \[ \] Implement StegoEngine: DCT Read/Write logic for JPEG resilience.  
* \[ \] Unit Tests: Verify Text \-\> Bits \-\> Pixels \-\> Text loop integrity.

**Phase 2: The Identity (The Handshake)**

* \[ \] Build QR Generator/Scanner (JSON parsing).  
* \[ \] **Feature:** Implement "Logo Overlay" logic with variable Error Correction (Level M vs Level H).  
* \[ \] Implement Root/Delegate Key Storage (Keychain vs. Local).  
* \[ \] Build "Contact Verification" UI.

**Phase 3: The Interface (The Product)**

* \[ \] Build Onboarding Flow (Standard vs. Ghost Mode, **explaining message sequencing constraints**).  
* \[ \] Build "Compose" and "Read" UI with **Native Share Sheet** support.  
* \[ \] Implement "Black Hole" moderation logic.

**Phase 4: Launch**

* \[ \] Create App Store Assets.  
* \[ \] File Export Compliance PDF.  
* \[ \] Final QA: Test Image Compression resilience (Instagram/Discord).
