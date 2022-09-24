create table m_purchase_task(
    id number(10),
    ad_client_id number(10) ,
    ad_org_id number(10) ,
    ownerid number(10) ,
    modifierid number(10) ,
    creationdate date ,
    modifieddate date ,
    isactive char(1), 
primary key (id));