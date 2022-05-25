#### **Module 5 - Computer Systems (2021-22)**  <img src="C:\Users\SarmahDK\AppData\Roaming\Typora\typora-user-images\image-20210602172621460.png" alt="image-20210602172621460" style="zoom:80%;" />

#### **Project**

##### **Final Product-Security by Design Checklist**


| Team ID: | Project Title: | Mentor(s): |
| -------- | -------------- | ---------- |
| Group 42 | T.W.I.P | Alexandru Matco & Marten Voorberg |

**					**

| **<span style="color:blue">Security Review for the system</span>** | **<span style="color:blue">Put ✔ if you have completed all the points as mentioned in Column 1.</span>** | **<span style="color:blue">Additional explanation</span>**
| ------------------------------------------------------------ | ------------------------------------------------------------ | ------------------------------------------------------------ |
| i) Need to ensure if the previous security control work for the system  |             ✔                                                 | The system makes use of salted sha512 algorithms to store passwords and security questions, these have been tested for their reliability and accuracy and are working adequately. |
| ii) If not, address the reason and discuss the mitigation plan among your team members |                                ✔                              | in addition to the encryption, we also implemented security questions on account creation that can be used to regain access to your account if you loose your password. The answers to these questions are also hashed and then stored |
| iii) Identify if any significant changes required for the system at the end. If yes, then ensure to update the impacted control decisions in the document as finalized in the previous phases. |       ✔                                                        | We decided against some previous ideas, such as USB identification keys and pipeline processing for hardware data. these implementation proved to be too difficult for the scope and time of this project |
| iv) Review and approve it with your TA                       |    ✔                                                            | |



**Note:** 

1. Be sure to accomplish the security review of the system according to the points. 
2. Review all the security checks of the product with your members and mentor(s).																																
