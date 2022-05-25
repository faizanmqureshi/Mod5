#### **Module 5 - Computer Systems (2021-22)**  <img src="C:\Users\SarmahDK\AppData\Roaming\Typora\typora-user-images\image-20210602172621460.png" alt="image-20210602172621460" style="zoom:80%;" />

#### **Project**

##### **Testing-Security by Design Checklist**


| Team ID: | Project Title: | Mentor(s): |
| -------- | -------------- | ---------- |
| Group 42 | T.W.I.P | Alexandru Matco & Marten Voorberg |

**					**

**Instructions:**

1. Refer to the below table. All the mentioned points are mandatory to perform for your application except point no. 4.

2. You should consider at least 2 vulnerabilities for each  criteria given in Column 'B', except point no. 4, 6, and 7.

3. The mitigation plan/solution should be considered for every  identified vulnerability. 

4. Make sure to review the document with your team members and mentor(s) before final submission.

5. This checklist should be in lined and submitted along with the Software Testing document. 

   



| **<span style="color:blue">Point</span>** | **<span style="color:blue">Source Code Review, Static and Dynamic Application Testing</span>** | **<span style="color:blue">Identified Vulnerabilities for testing (name them)</span>** | **<span style="color:blue">Put tick ✔ if you have completed all the points as mentioned in Column 1</span>** | **<span style="color:blue">Remarks, if any</span>** |
| ------------------------------------------ | ------------------------------------------------------------ | ------------------------------------------------------------ | ------------------------------------------------------------ | --------------------------------------------------- |
| 1.                                         | Application security vulnerabilities (e.g. Access Control, Injection, Authentication, Cross Site scripting, etc.) | Authentication                                  | ✓                                                              | To prevent data breaches, in additon to making use of hashing and salting, we use encryption between the front and backend of all password data. to reduce the risk of a "middle man attack"                   |
| 2.                                         | Weak security in functions (e.g. old encryption techniques, Hashing, Privileges assigned, Function error, etc.) | Storing of passwords                                                             | ✓                                                              | Passwords are salted and then hashed.                                                    |
| 3.                                         | Duplicate/unnecessary functions                              |                                                                | ✓                                                             |                                                     |
| 4.                                         | Analyzing Program (e.g. computation time, power consumption, etc.) (Optional) | Polling of sensor data                                                             | ✓ | The sensor is being polled each 10ns to get optimal readout reliability |
| 5.                                         | Address the remaining vulnerabilities of your application (manual) | - Accuracy of touchscreen <br> - Heat dissipation (due to high sensor polling rate)                                                             | ✓                                                              | The touchscreen accuracy is important as the screen is relativly small which results in small hitboxes for the input                                                    |
| 6.                                         | Make a mitigation plan/solution by listing down the vulnerabilities | 1. Authentication <br> 2. Database storage <br> 3. Sensor polling <br> 4. Touchscreen <br> 5. Heat dissipation                                                            | ✓                                                             | 1. Backup with security questions and account secured by PIN code <br> 2. Hashing of PIN codes <br> 3. Poll sensor at high rate and collect timings between polls as speed indication <br> 4. Larger hitboxes for buttons <br> 5. Open casing design                                                     |
| 7.                                         | Review with your team members and approve by your mentor(s). | -                                                            |                                                              |                                                     |



| Team members’ reviewed             | <span style="color:blue">(Ruter, yes), (Mila, Yes), (Daan, Yes), (Egor, Yes), (Novojit, Yes), (Faizan, Yes)</span> |
| ---------------------------------- | ------------------------------------------------------------ |
| **Mentors’ reviewed and verified** | <span style="color:blue">**(Mentor 1, yes), (Mentor 2, Yes)**</span> |

