#### **Module 5 - Computer Systems (2021-22)**  <img src="C:\Users\SarmahDK\AppData\Roaming\Typora\typora-user-images\image-20210602172621460.png" alt="image-20210602172621460" style="zoom:80%;" />

#### **Project**

##### **Security by Design Checklist**


| Team ID: | Project Title: | Mentor(s): |
| -------- | -------------- | ---------- |
| Group 42 | T.W.I.P | Alexandru Matco & Marten Voorberg |

**					**

**Instructions:**

1. Complete the sections in the below table and put a checkmark if you have done.
2. Think about your application and work on the sections accordingly.
3. Feel free to add extra requirements for reviewing security architecture and their countermeasures for your application, if needed. 
4. This document should be reviewed and approved by your team members and mentors before submission.
5. Make sure to submit this checklist along with the Software design document (SDD) on Canvas.
**					**


| Sr. No. | Review Security Architecture                                 | Put checkmark ✔ if you have completed the Review Security Architecture as suggested in the left column | Additional comments (if required) | Security Controls/Countermeasures                            | Put checkmark ✔ if you have completed the Security controls points as suggested in the left column | Additional comments (if required) |
| ------- | :----------------------------------------------------------- | ------------------------------------------------------------ | --------------------------------- | ------------------------------------------------------------ | ------------------------------------------------------------ | --------------------------------- |
| 1.      | <span style='color:blue'>**Check Trust Boundaries,** </span>***for example***, if you assign a higher privilege's level to someone to access a particular resource. |  ✔                                                            | Since our system can be used by multiple users but each of them have to login with their credentials (eg Username, pincode), the data stored will be on a local database inside the raspberry pi, only the user can access his or her data, thus there is no chance for any issues with other people getting access to the data. No outside connections would be possible, since we will disable bluetooth and disable outside wifi possibilites, thus keeping the system only local on the Pi. | <span style='color:blue'>**Check the prevention criteria**, </span>***for example***, if your personal information is identified then you can just go to forgot your pincode and change it by answering simple security question or you can delete that user and create new profile. This is a prevention criterion. | ✔                                                              | The user that is stored on the device has no significant impact on the security and live of the user if it is cracked, none of the data can be used to access legal or financial records or accounts |
| 2.      | <span style='color:blue'>**Identify data flows**, </span>***for example,*** if you read data from an untrusted source for your  application. | ✔                                                             | Since the password is only stored locally on the Raspberry Pi which is never connected to the internet, there is no chance for an attacker to crack the system. | <span style='color:blue'>**Check the mitigation criteria to reduce the impact of the risk/threat for the application.**</span> ***For example:*** Assume you have a database of users' passwords that are stored as a hash. Two users in the database who have the same password, they'll also have the same hash value. If the attacker identifies the hash value and its associated password, he'll be able to identify all the other passwords that have the same hash value. This risk can be mitigated by adding a randomly generated string, i.e. salt to each password in the database. | ✔                                                             | Our project add a string of random characters to the password before hashing as a salt.                          | 
| 3.      | **<span style='color:blue'>Entry and Exit points of the system and its components.</span>** | ✔                                                             |                                   | **<span style='color:blue'>Make a data flow diagram to visualize and understand the data flow, input, output points, and trust boundary.</span>** | ✔                                                             |                                   |
| 4.      | **<span style='color:blue'>Write the complete architecture in the SDD template. Review and approve among  yourselves and by your assigned mentor(s).</span>** | ✔                                                             |                                   | **<span style='color:blue'>Analyze the cost involved to implement the security controls (if any). </span>** | ✔                                                             |                                   |



| Team  members' reviewed:              | (Rutger, yes), (Mila, Yes), (Daan, Yes), (Novojit, Yes), (Egor, Yes), (Faizan, Yes)     |
| ------------------------------------- | ---------------------------------------- |
| **Mentor(s)  reviewed and verified:** | **(Mentor  1, Yes), (Mentor 2, Yes)** |

​																																											

​																																				
