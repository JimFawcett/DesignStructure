# DesignStructure
Contents of this repository contains short code illustrations of some design structure alternatives.  Code functionality is minimal so the details of the infrastracture are obvious.  The code simply counts lines of code in cited files.  So the input simply opens named files, compute counts lines, and output displays the results.  That lets us focus on partitioning,
ownership, and access.

Website documentation for this repository:<br />
  https://JimFawcett.github.io/DesignBites.html
  
Also, see the associated <a href="https://github.com/JimFawcett/DesignBites">DesignBites repository.

Figure 1. - Monolithic - simple structure, internally messy<br />
<img src="Design1.jpg#left" width="300" /><br />
<br />

Figure 2. - Factored - simpler internally, more parts<br />
<img src="Design2.jpg" width="400" /><br />
<br />

Figure 3. - Data Flow - more complex assembly, can see output before end of processing<br />
<img src="Design4.jpg" width="500" /><br />
<br />

Figure 4. - Type Erasure - reduces implementation dependencies<br />
<img src="Design5.jpg" width="700" /><br />
<br />

Figure 5. - PlugIn - Output loaded at run-time<br />
<img src="Design6.jpg" width="700" /><br />
<br />
  
